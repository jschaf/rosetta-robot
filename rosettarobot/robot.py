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
import json
import jsonpath_rw as jsonpath
import re
import subprocess
import tempfile


class CodeEntry(object):
    """Code information."""
    def __init__(self, path):
        "Create a code entry."
        self.path = path
        self.code = None
        with open(self.path, "r") as code_file:
            self.code = code_file.read()
        self.url_pattern = r"//.*?(http://rosettacode.org/wiki/[^\s]+)"
        self.url_regexp = re.compile(self.url_pattern)

    def extract_url(self):
        """Return the rosetta code url from the code."""
        for line in self.code.splitlines():
            matches = self._extract_url_string(line)
            if matches:
                return matches.group(1)

    def _extract_url_string(self, string):
        """Return the rosetta code url from one line of code."""
        return self.url_regexp.search(string)

    def generate_json_doc(self):
        """Return the rustdoc json for the CodeEntry."""
        json_str = ""
        with tempfile.NamedTemporaryFile() as temp:
            subprocess.call(["rustdoc", "-o", temp.name,
                             "--output-format", "json", self.path])
            json_str = temp.read().decode("utf-8")
        return json_str

    @staticmethod
    def _extract_docs(json_doc):
        """Extract module documentation for the CodeEntry."""
        json_str = json.loads(json_doc)
        jsonpath_expr = jsonpath.parse("$..module.attrs")
        matches = [match.value for match in jsonpath_expr.find(json_str)]
        if matches:
            matches = matches[0]

            module_docs = []
            for match in matches:
                if 'fields' in match and match['fields'][0] == 'doc':
                    module_docs.append(match['fields'][1])
            return module_docs
        else:
            return []

    def make_wiki_markup(self):
        """Return a string of wiki markup for RosettaCode.org."""
        json_doc = self.generate_json_doc()
        raw_docs = CodeEntry._extract_docs(json_doc)
        return "\n".join(raw_docs)


class RosettaEntry(object):
    """Hold information to post to RosettaCode.org."""
    def __init__(self, username, password):
        "Create a RosettaCode object."
        self.username = username
        self.password = password

    def authenticate(self):
        "Login to RosettaCode.org and obtain login context."
        pass

    def post_code(self, code_entry):
        """Post code_entry to RosettaCode.org."""
        pass


def main():
    """The entrypoint and command line interface for Rosetta Robot."""
    try:
        arguments = docopt.docopt(__doc__, version='Rosetta Robot 0.1')
        source_files = arguments['<src_file>']

        if arguments['upload']:
            print("Uploading")
        elif arguments['markup']:
            for src in source_files:
                code = CodeEntry(src)
                print("\n*** {}\n{}".format(src, code.make_wiki_markup()))

        elif arguments['check']:
            for src in source_files:
                code = CodeEntry(src)
                print("{}: {}".format(src, code.extract_url()))

            print("checking")

    except docopt.DocoptExit:
        print(docopt.DocoptExit)


if __name__ == '__main__':
    main()
