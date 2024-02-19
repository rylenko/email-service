#!/bin/python3

import os
from typing import Any
from pathlib import Path
from secrets import token_hex
from argparse import ArgumentParser
from json import dump, load, JSONDecodeError


BASE_DIR = Path(__file__).resolve().parent
DOCKER_DIR = BASE_DIR.joinpath("docker")
SERVICE_DIR = BASE_DIR.joinpath("email-service")

PORTS_PATH = BASE_DIR.joinpath("ports.json")

NODE_DOCKER_COMPOSE_PATH = DOCKER_DIR.joinpath("node.yml")
NODE_CONFIG_PATH = SERVICE_DIR.joinpath("node/config.json")
NODE_CONFIG_PATH_DOCKER = "/usr/src/node/config.json"

CLIENT_DOCKER_COMPOSE_PATH = DOCKER_DIR.joinpath("client.yml")
CLIENT_CONFIG_PATH = SERVICE_DIR.joinpath("client/config.json")
CLIENT_CONFIG_PATH_DOCKER = "/usr/src/client/config.json"

NODE_DEFAULT_CONFIG = {
	'password': None,
	'other_nodes': None,
}
CLIENT_DEFAULT_CONFIG = {
	'dark_theme': False,
	'proxy': None,
	'secret_key': token_hex(32),
}
DEFAULT_PORTS_CONFIG = {
	'node': 8000,
	'client': 8888,
}
JSON_INDENT = 4

parser = ArgumentParser()
parser.add_argument("component", type=str, choices=("node", "client"))


def dump_to_file(path: Path, obj: Any) -> None:
	with open(path, "w") as f:
		dump(obj, f, indent=JSON_INDENT)


def generate_node_config() -> None:
	print("Generating a config for a node...")
	dump_to_file(NODE_CONFIG_PATH, NODE_DEFAULT_CONFIG)


def generate_client_config() -> None:
	print("Generating a config for a client...")
	dump_to_file(CLIENT_CONFIG_PATH, CLIENT_DEFAULT_CONFIG)


def generate_ports() -> None:
	print("Generating a file with ports...")
	dump_to_file(PORTS_PATH, DEFAULT_PORTS_CONFIG)


def main() -> None:
	g = globals()
	c = parser.parse_args().component
	cu = c.upper()

	os.environ[cu + "_CONFIG_PATH_DOCKER"] = g[cu + "_CONFIG_PATH_DOCKER"]

	if not g[cu + "_CONFIG_PATH"].exists():
		g["generate_" + c + "_config"]()

	if not PORTS_PATH.exists():
		generate_ports()

	with open(PORTS_PATH) as f:
		try:
			os.environ[cu + "_PORT"] = str(load(f)[c])
		except (JSONDecodeError, ValueError, KeyError):
			print("Invalid %s port." % c)
			exit(1)

	docker_compose_path = g[cu + "_DOCKER_COMPOSE_PATH"]
	os.system("docker-compose -f " + str(docker_compose_path) + " up --build")


if __name__ == "__main__":
	main()
