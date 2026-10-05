"""Command-line entry point for PySure."""

import argparse

from pysure import __version__


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="pysure",
        description="Find Python versions compatible with a project's dependencies.",
    )
    parser.add_argument(
        "--version",
        action="version",
        version=f"%(prog)s {__version__}",
    )
    return parser


def main() -> None:
    build_parser().parse_args()


if __name__ == "__main__":
    main()