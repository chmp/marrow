self_path = __import__("pathlib").Path(__file__).parent.resolve()
python = __import__("shlex").quote(__import__("sys").executable)

__effect = lambda effect: lambda func: [func, effect(func.__dict__)][0]
cmd = lambda **kw: __effect(lambda d: d.setdefault("@cmd", {}).update(kw))
arg = lambda *a, **kw: __effect(lambda d: d.setdefault("@arg", []).append((a, kw)))

all_arrow_features = [
    # arrow-version:insert:     "arrow-{version}",
    "arrow-53",
    "arrow-52",
    "arrow-51",
    "arrow-50",
    "arrow-50",
    "arrow-49",
    "arrow-48",
    "arrow-47",
    "arrow-46",
    "arrow-45",
    "arrow-44",
    "arrow-43",
    "arrow-42",
    "arrow-41",
    "arrow-40",
    "arrow-39",
    "arrow-38",
    "arrow-37",
]
all_arrow2_features = ["arrow2-0-17", "arrow2-0-16"]
default_features = f"{all_arrow2_features[0]},{all_arrow_features[0]}"

CHECKS_PLACEHOLDER = "<<< checks >>>"

workflow_test_template = {
    "name": "Test",
    "on": {
        "workflow_dispatch": {},
        "pull_request": {
            "branches": ["main", "develop-*"],
            "types": [
                "opened",
                "edited",
                "reopened",
                "ready_for_review",
                "synchronize",
            ],
        },
    },
    "env": {"CARGO_TERM_COLOR": "always"},
    "jobs": {
        "test": {
            "runs-on": "ubuntu-latest",
            "steps": [
                {"uses": "actions/checkout@v4"},
                {"name": "rustc", "run": "rustc --version"},
                {"name": "cargo", "run": "cargo --version"},
                CHECKS_PLACEHOLDER,
            ],
        }
    },
}

workflow_release_template = {
    "name": "Release",
    "on": {
        "release": {"types": ["published"]},
    },
    "env": {"CARGO_TERM_COLOR": "always"},
    "jobs": {
        "release": {
            "runs-on": "ubuntu-latest",
            "env": {
                "CARGO_REGISTRY_TOKEN": "${{ secrets.CARGO_REGISTRY_TOKEN }}",
            },
            "steps": [
                {"uses": "actions/checkout@v4"},
                {"name": "rustc", "run": "rustc --version"},
                {"name": "cargo", "run": "cargo --version"},
                CHECKS_PLACEHOLDER,
                {
                    "name": "Publish to crates.io",
                    "working-directory": "marrow",
                    "run": "cargo publish",
                },
            ],
        }
    },
}


@cmd(help="Run all common development tasks before a commit")
@arg("--backtrace", action="store_true", default=False)
def precommit(backtrace=False):
    update_workflows()

    format()
    check()
    test(backtrace=backtrace)


@cmd(help="Update the github workflows")
def update_workflows():
    _update_workflow(
        self_path / ".github" / "workflows" / "test.yml",
        workflow_test_template,
    )

    _update_workflow(
        self_path / ".github" / "workflows" / "release.yml",
        workflow_release_template,
    )


def _update_workflow(path, template):
    import copy, json

    workflow = copy.deepcopy(template)

    for job in workflow["jobs"].values():
        steps = []
        for step in job["steps"]:
            if step == CHECKS_PLACEHOLDER:
                steps.extend(_generate_workflow_check_steps())

            else:
                assert isinstance(step, dict)
                steps.append(step)

        job["steps"] = steps

    print(f":: update {path}")
    with open(path, "wt", encoding="utf8", newline="\n") as fobj:
        json.dump(workflow, fobj, indent=2)


def _generate_workflow_check_steps():
    yield {"name": "Check", "run": "cargo check"}
    for feature in (*all_arrow2_features, *all_arrow_features):
        yield {
            "name": f"Check {feature}",
            "run": f"cargo check --features {feature}",
        }

    yield {
        "name": "Check format",
        "run": "cargo fmt --check",
    }
    yield {
        "name": "Build",
        "run": f"cargo build --features {default_features}",
    }
    yield {
        "name": "Test",
        "run": f"cargo test --features {default_features}",
    }


@cmd(help="Format the code")
def format():
    _sh(f"{python} -m black {_q(__file__)}")
    _sh("cargo fmt")


@cmd(help="Run the linters")
@arg("--all", action="store_true")
def check(all=False):
    check_cargo_toml()
    _sh(f"cargo check --features {default_features}")
    _sh(f"cargo clippy --features {default_features}")

    if all:
        for features in (*all_arrow2_features, *all_arrow_features):
            _sh(f"cargo check --features {features}")


@cmd(help="Run the tests")
@arg(
    "--backtrace",
    action="store_true",
    default=False,
    help="If given, print a backtrace on error",
)
@arg(
    "--full",
    action="store_true",
    default=False,
    help="If given, run all feature combinations",
)
@arg("test_name", nargs="?", help="Filter of test names")
def test(test_name=None, backtrace=False, full=False):
    if not full:
        feature_selections = [f"--features {default_features}"]

    else:
        feature_selections = [
            (
                f"--features {', '.join(arrow_feature + arrow2_feature)}"
                if arrow_feature or arrow2_feature
                else ""
            )
            for arrow_feature in [[], *([feat] for feat in all_arrow_features)]
            for arrow2_feature in [[], *([feat] for feat in all_arrow2_features)]
        ]

    for feature_selection in feature_selections:
        _sh(
            f"""
                cargo test
                    {feature_selection}
                    {_q(test_name) if test_name else ''}
            """,
            env=({"RUST_BACKTRACE": "1"} if backtrace else {}),
        )


@cmd()
def check_cargo_toml():
    import tomli

    print(":: check Cargo.toml")
    with open(self_path / "marrow" / "Cargo.toml", "rb") as fobj:
        config = tomli.load(fobj)

    for label, features in [
        (
            "docs.rs configuration",
            config["package"]["metadata"]["docs"]["rs"]["features"],
        ),
        *[
            (f"test {target['name']}", target["required-features"])
            for target in config.get("test", [])
        ],
        *[
            (f"bench {target['name']}", target["required-features"])
            for target in config.get("bench", [])
        ],
    ]:
        actual_features = sorted(features)
        expected_features = sorted(default_features.split(","))

        if actual_features != expected_features:
            raise ValueError(
                f"Invalid {label}. "
                f"Expected: {expected_features}, found: {actual_features}"
            )

    # TODO: check the features / dependencies
    for feature in all_arrow_features:
        *_, version = feature.partition("-")

        actual_feature_def = sorted(config["features"][feature])
        expected_feature_def = sorted(
            [
                f"dep:arrow-array-{version}",
                f"dep:arrow-schema-{version}",
                f"dep:arrow-data-{version}",
                f"dep:arrow-buffer-{version}",
            ]
        )

        if actual_feature_def != expected_feature_def:
            raise ValueError(
                f"Invalid feature definition for {feature}. "
                f"Expected: {expected_feature_def}, found: {actual_feature_def}"
            )

        for component in ["arrow-array", "arrow-schema", "arrow-data", "arrow-buffer"]:
            expected_dep = {
                "package": component,
                "version": version,
                "optional": True,
                "default-features": False,
            }
            actual_dep = config["dependencies"].get(f"{component}-{version}")

            if actual_dep is None:
                raise ValueError(f"Missing dependency {component}-{version}")

            if actual_dep != expected_dep:
                raise ValueError(
                    f"Invalid dependency {component}-{version}. "
                    f"Expected: {expected_dep}, found: {actual_dep}"
                )

        for name, dep in config["dependencies"].items():
            if dep.get("default-features", True):
                raise ValueError(f"Default features for {name} not deactivated")


@cmd(help="Generate the documentation")
@arg("--private", action="store_true", default=False)
@arg("--open", action="store_true", default=False)
def doc(private=False, open=False):
    _sh(
        f"""
            cargo doc
                --features {default_features}
                {'--document-private-items' if private else ''}
                {'--open' if open else ''}
        """,
    )


@cmd(help="Add a new arrow version")
@arg("version")
def add_arrow_version(version):
    import re

    if _sh("git diff-files --quiet", check=False).returncode != 0:
        print(
            "WARNING: potentially destructive changes. "
            "Please stage or commit the working tree first."
        )
        raise SystemExit(1)

    for p in [
        self_path / "x.py",
        *self_path.glob("*/**/*.rs"),
        *self_path.glob("*/**/*.toml"),
    ]:
        content = p.read_text()
        if "arrow-version" not in content:
            continue

        print(f"process {p}")
        new_content = []
        include_next = True
        for line in content.splitlines():
            if (
                m := re.match(r"^.*(//|#) arrow-version:(replace|insert): (.*)$", line)
            ) is not None:
                new_content.append(line)
                new_content.append(
                    m.group(3).format_map({"version": version, "\\n": "\n"})
                )
                include_next = m.group(2) != "replace"

            else:
                if include_next:
                    new_content.append(line)

                include_next = True

        with open(p, "wt", newline="\n", encoding="utf-8") as fobj:
            fobj.write("\n".join(new_content))

    format()
    update_workflows()


_sh = lambda c, env=(), **kw: __import__("subprocess").run(
    [args := __import__("shlex").split(c.replace("\n", " ")), print("::", *args)][0],
    **{
        "check": True,
        "cwd": self_path,
        "encoding": "utf-8",
        "env": {**__import__("os").environ, **dict(env)},
        **kw,
    },
)
_q = lambda arg: __import__("shlex").quote(str(arg))

if __name__ == "__main__":
    _sps = (_p := __import__("argparse").ArgumentParser()).add_subparsers()
    for _f in (f for _, f in sorted(globals().items()) if hasattr(f, "@cmd")):
        _kw = {"name": _f.__name__.replace("_", "-"), **getattr(_f, "@cmd")}
        (_sp := _sps.add_parser(**_kw)).set_defaults(_=_f)
        [_sp.add_argument(*a, **kw) for a, kw in reversed(getattr(_f, "@arg", []))]
    (_a := vars(_p.parse_args())).pop("_", _p.print_help)(**_a)
