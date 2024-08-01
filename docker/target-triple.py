#!/usr/bin/env python

import argparse
from dataclasses import dataclass
from typing import Any, Literal, Self, Type


CMD_RUST_PLATFORM_T = Literal["rust-platform"]
CMD_CROSSBUILD_ESS_T = Literal["crossbuild-essential"]
CMD_RUST_PLATFORM: CMD_RUST_PLATFORM_T = "rust-platform"
CMD_CROSSBUILD_ESS: CMD_CROSSBUILD_ESS_T = "crossbuild-essential"


@dataclass(frozen=True)
class Platform:
    os: str
    arch: str
    variant: str | None

    def rust_platform(self) -> str:
        # https://github.com/GoogleContainerTools/distroless
        # platform is expected to be one of:
        #     linux/386, linux/amd64, linux/arm/v6, linux/arm/v7, linux/arm64,
        #     linux/ppc64le, linux/s390x
        # https://doc.rust-lang.org/nightly/rustc/platform-support.html
        assert self.os == "linux"
        arch = self.arch
        match (self.arch, self.variant):
            case ("arm64", None):
                arch = "aarch64"
                vendor = "unknown"
                abi = "gnu"
            case ("arm", "v6"):
                arch = "arm"
                vendor = "unknown"
                abi = "gnueabihf"
            case ("arm", "v7"):
                arch = "armv7"
                vendor = "unknown"
                abi = "gnueabihf"
            case ("amd64", None):
                arch = "x86_64"
                vendor = "unknown"
                abi = "gnu"
            case ("386", None):
                # FIXME: is this correct?
                arch = "x86_64"
                vendor = "unknown"
                abi = "gnu"
            case ("ppc64le", None):
                arch = "powerpc64le"
                vendor = "unknown"
                abi = "gnu"
            case ("s390x", None):
                arch = "s390x"
                vendor = "unknown"
                abi = "gnu"
            case _:
                assert False, f"unexpected input: {self}"
        return f"{arch}-{vendor}-{self.os}-{abi}"

    def crossbuild_essential(self) -> str:
        match self.arch:
            case "amd64" | "386":
                return "crossbuild-essential-amd64"
            case "arm64" | "arm":
                return "crossbuild-essential-arm64"
            case "ppc64le":
                return "crossbuild-essential-ppc64el"
            case "s390x":
                return "crossbuild-essential-s390x"
            case _:
                assert False, f"unexpected input: {self}"

    @classmethod
    def from_args(cls, args: Any) -> Self:
        os = getattr(args, "os", None)
        assert isinstance(os, str), f"unexpected arguments: {args}"
        arch = getattr(args, "arch", None)
        assert isinstance(arch, str), f"unexpected arguments: {args}"
        variant = getattr(args, "variant", None) or None
        assert isinstance(variant, str) or variant is None, f"unexpected arguments: {args}"
        return cls(os, arch, variant)


@dataclass(frozen=True)
class Args:
    subcommand: CMD_RUST_PLATFORM_T | CMD_CROSSBUILD_ESS_T
    platform: Platform

    @classmethod
    def rust_platform_args(cls, args: Any) -> Self:
        platform = Platform.from_args(args)
        return cls(CMD_RUST_PLATFORM, platform)

    @classmethod
    def gcc_pkgname_args(cls, args: Any) -> Self:
        platform = Platform.from_args(args)
        return cls(CMD_CROSSBUILD_ESS, platform)



def prepare_subcommand_parser(parser: argparse.ArgumentParser) -> None:
    parser.add_argument("--os", required=True)
    parser.add_argument("--arch", required=True)
    parser.add_argument("--variant", required=False)


def parse_args() -> Args:
    parser = argparse.ArgumentParser("target-triple.py")
    subparsers = parser.add_subparsers()
    rust_platform_parser = subparsers.add_parser(CMD_RUST_PLATFORM)
    prepare_subcommand_parser(rust_platform_parser)
    rust_platform_parser.set_defaults(func=Args.rust_platform_args)
    gcc_pkgname_parser = subparsers.add_parser(CMD_CROSSBUILD_ESS)
    prepare_subcommand_parser(gcc_pkgname_parser)
    gcc_pkgname_parser.set_defaults(func=Args.gcc_pkgname_args)
    args = parser.parse_args()
    ret = args.func(args)
    assert isinstance(ret, Args)
    return ret


def proc_args(args: Args) -> str:
    if args.subcommand == CMD_RUST_PLATFORM:
        return args.platform.rust_platform()
    assert args.subcommand == CMD_CROSSBUILD_ESS
    return args.platform.crossbuild_essential()


def main() -> None:
    args = parse_args()
    result = proc_args(args)
    print(result)


if __name__ == "__main__":
    main()
