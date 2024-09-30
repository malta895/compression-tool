TMP_DIR=$(mktemp -d)
WINRAT_BIN="target/release/compression-tool"

set -e

(
    echo "file gzip_size winrar_size ratio"
    for file in fixtures/*; do
        compressed_file="$TMP_DIR/$(basename $file)"
        gzip -c $file > "$compressed_file.gz"
        $WINRAT_BIN -i $file -o "$compressed_file.rar"

        gzip_size=$(du "$compressed_file.gz" | cut -f1)
        winrat_size=$(du "$compressed_file.rar" | cut -f1)

        echo "$file $(du -h "$compressed_file.gz" | cut -f1) $(du -h "$compressed_file.rar" | cut -f1) $(echo "scale=4; $gzip_size / $winrat_size" | bc)"
    done
) | column -t