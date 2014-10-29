"""
rosettarobot

Upload source files to RosettaCode.org

Usage:
  rosettarobot upload <src_file>...
  rosettarobot markup [--github|--mediawiki] <src_file>...
  rosettarobot check [--out-file=<rosetta-check.txt>] <src_file>...

Options:
  -h --help         Show this screen.
  -i --interactive  Run in interactive mode.
  -n --dry-run      Only print what would have happened.
  -q --quiet        Be very, very quiet.
  -v --verbose      Be very, very unquiet.
"""


import docopt
import jinja2
import json
import jsonpath_rw as jsonpath
import re
import subprocess
import tempfile

import requests
import lxml.html
import urllib

ROSETTA_URL = "http://rosettacode.org"


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
        docs = "\n".join(raw_docs)

        jinja_env = jinja2.Environment(
            loader=jinja2.PackageLoader('rosettarobot', 'templates'),
            block_start_string='<%',
            block_end_string='%>',
            variable_start_string='%%',
            variable_end_string='%%',
            comment_start_string='<#',
            comment_end_string='#>',
        )

        template = jinja_env.get_template("mediawiki_template.jinja2")
        return template.render(header=None,
                               documentation=docs,
                               code=self.code,
                               output=None)

    def make_github_markup(self):
        json_doc = self.generate_json_doc()
        raw_docs = CodeEntry._extract_docs(json_doc)
        docs = "\n".join(raw_docs)

        jinja_env = jinja2.Environment(
            loader=jinja2.PackageLoader('rosettarobot', 'templates'),
        )

        template = jinja_env.get_template("github_template.jinja2")
        return template.render(header=None,
                               documentation=docs,
                               code=self.code,
                               output=None)


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


class RosettaCode(object):
    """Get information from RosettaCode.org"""

    @staticmethod
    def extract_edit_section_url(tree):
        """Given an lxml tree of a RosettaCode page, return the edit url."""
        url_xpath = '//*[@id="Rust"]/preceding-sibling::span/a/@href'
        url_list = tree.xpath(url_xpath)
        if url_list:
            # xpath returns a list
            edit_section_url = url_list[0]
            return urllib.parse.urljoin(ROSETTA_URL, edit_section_url)
        else:
            return None

    @staticmethod
    def extract_markup(tree):
        """Given an lxml etree of an RosettaCode edit page, return the
markup."""
        markup_xpath = '//textarea/text()'
        textarea_list = tree.xpath(markup_xpath)
        if textarea_list:
            # xpath returns a list
            markup = textarea_list[0]
            return markup
        else:
            return None

    @staticmethod
    def get_rosetta_code_markup(url):
        """Given a RosettaCode.org url, return the Wikimedia markup for that
page."""
        rosetta_request = requests.get(url)
        rosetta_tree = lxml.html.fromstring(rosetta_request.text)
        edit_url = RosettaCode.extract_edit_section_url(rosetta_tree)
        markup_request = requests.get(edit_url)
        markup_tree = lxml.html.fromstring(markup_request.text)
        return RosettaCode.extract_markup(markup_tree)


def main():
    """The entrypoint and command line interface for Rosetta Robot."""
    try:
        arguments = docopt.docopt(__doc__, version='Rosetta Robot 0.1')
        source_files = arguments['<src_file>']

        if arguments['upload']:
            print("Uploading")
        elif arguments['markup']:
            markup_fn = CodeEntry.make_wiki_markup

            if arguments['--github']:
                print("Using github flavored markdown syntax.")
                markup_fn = CodeEntry.make_github_markup

            elif arguments['--mediawiki']:
                print("Using mediawiki syntax.")

            else:
                print("No syntax declared, using mediawiki syntax.")

            for src in source_files:
                code = CodeEntry(src)
                print("\n*** {}\n{}".format(src, markup_fn(code)))

        elif arguments['check']:
            for src in source_files:
                code = CodeEntry(src)
                print("{}: {}".format(src, code.extract_url()))

            print("checking")

    except docopt.DocoptExit:
        print(docopt.DocoptExit)


if __name__ == '__main__':
    main()
