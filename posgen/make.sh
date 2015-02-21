#!/bin/sh

cargo build || exit

for def in $(find ../src -name "def.toml"); do
    dir="$(dirname $def)"
    test -e "$dir/$1"
    os_existed=$?
    mkdir -p "$dir/$1"
    if [ $os_existed -ne 0 ]; then
        sed '/^$/,$d' "$dir/linux/mod.rs" > "$dir/$1/mod.rs"
        awk '/^mod os;$/ { if (set == 0) {
            print "mod os;\n"
            print "#[cfg(target_os = \"'$1'\")]"
            print "#[path = \"'$1'/mod.rs\"]"
            set = 1
        } } { print $0 }' "$dir/mod.rs" > "$dir/tmp.rs"
        mv "$dir/tmp.rs" "$dir/mod.rs"
    fi
    dest="$dir/$1/$2.rs"
    if [ ! -e $dest ]; then
        echo "" >> "$dir/$1/mod.rs"
        echo "#[cfg(target_arch = \"$2\")]" >> "$dir/$1/mod.rs"
        echo "#[path = \"$2.rs\"]" >> "$dir/$1/mod.rs"
        echo "mod arch;"  >> "$dir/$1/mod.rs"
    fi
    ./target/posgen "$def" > "$dest"
done
