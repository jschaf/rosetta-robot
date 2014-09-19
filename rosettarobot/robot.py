"""
rosettarobot

Upload source files to RosettaCode.org

Usage:
  rosettarobot upload <src_file>...
  rosettarobot markup <src_file>...
  rosettarobot check [--out-file=<rosetta-check.txt>] <src_file>...

Options:
  -h --help         Show this screen.
  -i --interactive  Run in interactive mode.
  -n --dry-run      Only print what would have happened.
  -q --quiet        Be very, very quiet.
  -v --verbose      Be very, very unquiet.
"""


import docopt
import re

class Code_Entry(object):
    def __init__(self, path):
        "Create a code entry."
        self.path = path
        self.code = None
        with open(self.path, "r") as f:
            self.code = f.read()
        self.url_pattern = "//.*?(http://rosettacode\.org/wiki/[^\s]+)"
        self.url_regexp = re.compile(self.url_pattern)


    def extract_url(self):
        for line in self.code.splitlines():
            matches = self._extract_url_string(line)
            if matches:
                return matches.group(1)


    def _extract_url_string(self, string):
        return self.url_regexp.search(string)


    def make_wiki_markup(self):
        return "wiki markup"


class Rosetta_Entry(object):
    def __init__(self, username, password):
        "Create a RosettaCode object."
        self.username = username
        self.password = password


    def authenticate(self):
        "Login to RosettaCode.org and obtain login context."
        return "CONTEXT"


def main():
    try:
        arguments = docopt.docopt(__doc__, version='Rosetta Robot 0.1')
        source_files = arguments['<src_file>']

        if arguments['upload']:
            print("Uploading")
        elif arguments['markup']:
            print("marking up")
        elif arguments['check']:
            for src in source_files:
                code = Code_Entry(src)
                print("{}: {}".format(src, code.extract_url()))

            print("checking")

    except docopt.DocoptExit as e:
        print(e)


if __name__ == '__main__':
    main()
