from rosettarobot.robot import RosettaCode
import lxml

EMPTY_HTML = '''<!doctype html>
    <html lang=en>
    <head>
    <meta charset=utf-8>
    <title>blah</title>
    </head>
    <body>
    <p>I'm the content</p>
    </body>
    </html>'''


def test_extract_edit_section_url():
    html = '''<!DOCTYPE html>
<html>
<head><title></title></head>
<body>
    <h2>
      <span class="editsection">
       [<a href="/mw/index.php?title=100_doors&amp;action=edit&amp;section=190"
    title="Edit section: Rust">edit</a>]
      </span>
    <span class="mw-headline" id="Rust">
      <a href="/wiki/Category:Rust" title="Category:Rust">Rust</a></span>
    </h2>
</body>
</html>'''
    tree = lxml.html.fromstring(html)
    url = RosettaCode.extract_edit_section_url(tree)
    assert url == 'http://rosettacode.org/mw/index.php?title=100_doors&action=edit&section=190'


def test_no_edit_section_url():
    html = EMPTY_HTML
    tree = lxml.html.fromstring(html)
    url = RosettaCode.extract_edit_section_url(tree)
    assert url is None


def test_extract_markup():
    html = '''<!DOCTYPE html>
<html>
<head><title></title></head>
<body>
    <textarea accesskey="," cols="80" dir="ltr" id="wpTextbox1" lang="en" name=
    "wpTextbox1" rows="25" style="" tabindex="1">YOLO</textarea>
</body>
</html>'''
    tree = lxml.html.fromstring(html)
    markup = RosettaCode.extract_markup(tree)
    assert markup == "YOLO"


def test_extract_markup_no_textarea():
    html = EMPTY_HTML
    tree = lxml.html.fromstring(html)
    markup = RosettaCode.extract_markup(tree)
    assert markup is None


def test_get_rosetta_code_markup():
    pass
