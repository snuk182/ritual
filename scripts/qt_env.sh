#!/bin/sh

QT_DIR="$1"
echo "QT_DIR=$QT_DIR"
export PATH="$QT_DIR/bin:$PATH"
export LD_LIBRARY_PATH="$QT_DIR/lib:$LD_LIBRARY_PATH"
export QT_QPA_PLATFORM_PLUGIN_PATH="$QT_DIR/plugins"
export QML2_IMPORT_PATH="$QT_DIR/qml"
