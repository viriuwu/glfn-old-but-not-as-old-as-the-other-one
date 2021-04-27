#!/usr/bin/env bash

SCRIPT_PATH=$(readlink -f "$0")
SCRIPT_DIR=$(dirname "$SCRIPT_PATH")

REPO="KhronosGroup/OpenGL-Registry"
GL_XML="https://raw.githubusercontent.com/$REPO/master/xml/gl.xml"

true && \
    printf "> Re-downloading \`gl.xml\`\n" && \
        curl -sSf -o "$SCRIPT_DIR/gl.xml" "$GL_XML" && \
    printf "> Logging SHA (GitHub API)\n" && \
        curl -sSf -H "Accept: application/vnd.github.v3+json" "https://api.github.com/repos/$REPO/commits/HEAD" | \
        perl -e 'use JSON; my $resp = decode_json(do { local $/; <STDIN> }); print $resp->{"sha"};' | \
        printf "repo $REPO\nsha $(cat -)\n" \
            > "$SCRIPT_DIR/commit"

printf "Done!\n"
