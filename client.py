# noinspection PyUnresolvedReferences
import readline
import shlex
from argparse import ArgumentParser, REMAINDER
from socket import create_connection, socket, AF_INET, SOCK_DGRAM, SHUT_WR

import sys

PREFIX = b"^^^^"
SUFFIX = b"$$$$\n"


def port_type(n):
    n = int(n)
    if n > 0:
        return n
    else:
        raise ValueError("Invalid port!")


def main(args):
    cc = CommandClient(args.host, args.port, args.udp)
    while True:
        prompt = "# "
        try:
            cc.run_command(input(prompt))
        except KeyboardInterrupt:
            print()
        except EOFError:
            print()
            sys.exit(0)


class CommandClient:
    def __init__(self, host: str, port: int, udp: bool = False):
        self.host = host
        self.port = port
        self.udp = udp

    def connect(self) -> socket:
        if self.udp:
            sock = socket(AF_INET, SOCK_DGRAM)
            sock.connect((self.host, self.port))
            return sock
        else:
            return create_connection((self.host, self.port))

    def send_command(self, command: str) -> bytes:
        conn = self.connect()
        conn.send(PREFIX + command.encode() + SUFFIX)
        conn.shutdown(SHUT_WR)
        return conn.recv(4096)

    def handle_cmd(self, args):
        print(self.send_command(' '.join(args.cmds)))

    def run_command(self, command):
        if not command:
            print()
            return
        parser = ArgumentParser()
        parser.add_argument("-n", "--no-fork", action="store_true")
        subparsers = parser.add_subparsers(dest='command')
        subparsers.required = True
        exec_cmd_parser = subparsers.add_parser("cmd")
        exec_cmd_parser.set_defaults(func=self.handle_cmd)
        exec_cmd_parser.add_argument('cmds', nargs=REMAINDER)
        try:
            args = parser.parse_args(shlex.split(command))
        except SystemExit:
            print()
            return
        args.func(args)


def parse_args():
    parser = ArgumentParser()
    parser.add_argument("host")
    parser.add_argument("port", type=port_type)
    parser.add_argument("-u", "--udp", action="store_true")
    return parser.parse_args()


if __name__ == "__main__":
    main(parse_args())
