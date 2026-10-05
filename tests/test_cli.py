from pysure.cli import build_parser


def test_cli_version(capsys):
    try:
        build_parser().parse_args(["--version"])
    except SystemExit as error:
        assert error.code == 0

    assert capsys.readouterr().out.strip() == "pysure 0.1.0"