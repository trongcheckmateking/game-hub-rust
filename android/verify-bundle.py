"""Validate the release manifest after dependency manifests have been merged."""
import sys
import xml.etree.ElementTree as ET
m=ET.parse(sys.argv[1]).getroot()
a='{http://schemas.android.com/apk/res/android}'
app=m.find('application')
assert app is not None and m.get('package')=='dev.trong.gamehub'
assert int(m.find('uses-sdk').get(a+'targetSdkVersion'))>=36
assert app.get(a+'debuggable','false')=='false'
assert app.get(a+'allowBackup')=='false' and app.get(a+'usesCleartextTraffic')=='false'
sensitive=[e.get(a+'name') for e in m if e.tag.startswith('uses-permission') and e.get(a+'name','').startswith('android.permission.')]
assert not sensitive, sensitive
