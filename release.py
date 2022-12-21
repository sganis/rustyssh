#!/usr/bin/python3
#
# Deployment script
# Update version information, commit, add tag with version, and push.
# CI will deploy when a new tag is added.
#
import sys
import os
import subprocess
import re

DIR = os.path.dirname(os.path.realpath(__file__))

def run(command):
    print('Running: {0}'.format(command))
    proc = subprocess.Popen(command, bufsize=0, shell=True,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    stdout, stderr = proc.communicate()
    stdout = str(stdout.decode('utf-8')).strip()
    stderr = str(stderr.decode('utf-8')).strip()
    print(stdout)
    if stderr:
        print(f'stderr: {stderr}')
    return stdout, stderr

def update_version(version):
    # update tauri version
    updated = False
    file = f'{DIR}/src-tauri/tauri.conf.json'
    newlines = []
    lines = open(file).readlines()
    for line in lines:
        m = re.match(r'^(.+)\"version\": \"([0-9\.]+)\"', line)
        if m:
            line = f'{m.group(1)}\"version\": \"{version}\"\n'
            updated = True
        newlines.append(line)
    assert updated
    open(file, 'w').writelines(newlines)
    
    # update package.json
    updated = False
    file = f'{DIR}/package.json'
    newlines = []
    lines = open(file).readlines()
    for line in lines:
        m = re.match(r'^(.+)\"version\": \"([0-9\.]+)\",', line)
        if m:
            line = f'{m.group(1)}\"version\": \"{version}\",\n'
            updated = True
        newlines.append(line)
    assert updated
    open(file, 'w').writelines(newlines)
    print(f'Updated to version: {version}')
    return True

def main():
    # get version
    version = open(f'{DIR}/version.txt', 'r').read().strip()
    assert version
    
    # update version files in source code
    if not update_version(version):
        print('cannot update version in source code.')
        sys.exit(1)

    tag = 'v{0}'.format(version)

    # pull
    run('git pull')

    # commit
    run('git commit -am "{0}"'.format(tag))

    # tag
    run('git tag -a {0} -m "{0}"'.format(tag))

    # push
    run('git push --follow-tags')


if __name__ == '__main__':
    if len(sys.argv) > 2 and sys.argv[1] == '--delete-tag':
        tag = sys.argv[2]
        run(f'git tag -d {tag}')
        run(f'git push --delete origin {tag}')
    else:
        main()
        print('done.')

