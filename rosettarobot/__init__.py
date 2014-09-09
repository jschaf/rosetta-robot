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


class Code_Entry(object):
    def __init__(self, path):
        "Create a code entry."
        self.path = path


    def extract_url(self):
        return "URL"


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
        arguments = docopt.docopt(__doc__)
        source_files = arguments['src_file']

    except docopt.DocoptExit as e:
        print(e.message)


if __name__ == '__main__':
    main()
