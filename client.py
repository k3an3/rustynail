import readline
import sys
from argparse import ArgumentParser


def port(n):
    n = int(n)
    if n > 0:
        return n
    else:
        raise ValueError("Invalid port!")

def run_command(command):
    parser = ArgumentParser()
    parser.add_argument("-n", "--no-fork", action="store_true")
    subparsers = parser.add_subparsers("command", required=True)
    exec_cmd_parser = subparsers.add_parser("cmd")
    

def main(args):
    while True:
        prompt = "# "
        try:
            run_command(input(prompt))
        except KeyboardInterrupt:
            print()
        except EOFError:
            print()
            sys.exit(0)


def parse_args():
    parser = ArgumentParser()
    parser.add_argument("host")
    parser.add_argument("port", type=port)
    parser.add_argument("-u", "--udp", action="store_true")
    return parser.parse_args()

if __name__ == "__main__":
    main(parse_args())
