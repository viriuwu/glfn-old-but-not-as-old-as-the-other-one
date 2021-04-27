#!/usr/bin/env bash

SCRIPT_PATH=$(readlink -f "$0")
SCRIPT_DIR=$(dirname "$SCRIPT_PATH")

GL_XML='https://raw.githubusercontent.com/KhronosGroup/OpenGL-Registry/master/xml/gl.xml'

echo 'Updating XML registry...' && \
    echo '> Re-downloading `gl.xml`' && curl -sSf -o "$SCRIPT_DIR/gl.xml" "$GL_XML" && \
    echo 'Done!'
