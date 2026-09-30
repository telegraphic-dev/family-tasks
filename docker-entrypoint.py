#!/usr/bin/env python3
"""Configure local-only public-origin hints before starting Reboot."""

import importlib.util
import json
import os
import sys
from pathlib import Path
from urllib.parse import urlsplit


def public_origin() -> str | None:
    value = os.environ.get("FAMILY_TASKS_PUBLIC_URL")
    if not value:
        return None

    parsed = urlsplit(value)
    if (
        parsed.scheme != "https"
        or not parsed.netloc
        or parsed.username
        or parsed.password
        or parsed.path not in ("", "/")
        or parsed.query
        or parsed.fragment
    ):
        raise SystemExit(
            "FAMILY_TASKS_PUBLIC_URL must be an HTTPS origin without a path, query, or credentials."
        )
    return value.rstrip("/")


def configure_root_page(origin: str) -> None:
    spec = importlib.util.find_spec("reboot.rootpage")
    if spec is None or not spec.submodule_search_locations:
        raise SystemExit("Could not locate Reboot's root-page bundle.")

    bundle = Path(next(iter(spec.submodule_search_locations))) / "bundle.js"
    content = bundle.read_text()
    host = urlsplit(origin).netloc
    old_expected_host = "const expectedHost = `localhost:${appPort}`;"
    old_command = (
        "const launchCommand = `npx @mcpjam/inspector@${MCPJAM_VERSION} "
        "--url http://localhost:${appPort}/mcp --oauth`;"
    )
    new_expected_host = f"const expectedHost = {json.dumps(host)};"
    new_command = (
        "const launchCommand = "
        + json.dumps(f"npx @mcpjam/inspector@2.23.3 --url {origin}/mcp --oauth")
        + ";"
    )

    if old_expected_host not in content or old_command not in content:
        raise SystemExit("Unsupported Reboot root-page bundle; refusing to patch MCPJam hints.")

    bundle.write_text(
        content.replace(old_expected_host, new_expected_host, 1).replace(old_command, new_command, 1)
    )


def main() -> None:
    origin = public_origin()
    if origin:
        configure_root_page(origin)
    os.execvp("/usr/bin/tini", ["/usr/bin/tini", "--", *sys.argv[1:]])


if __name__ == "__main__":
    main()
