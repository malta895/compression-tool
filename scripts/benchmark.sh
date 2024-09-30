TMP_DIR=$(mktemp -d)
WINRAT_BIN="target/release/compression-tool"

set -e

(
    echo "file file_size gzip_size winrar_size gzip_file_ratio winrar_file_ratio gzip_winrar_ratio"
    for file in fixtures/*; do
        compressed_file="$TMP_DIR/$(basename $file)"
        gzip -c $file > "$compressed_file.gz"
        $WINRAT_BIN -i $file -o "$compressed_file.rar"

        file_size=$(du $file | cut -f1)
        gzip_size=$(du "$compressed_file.gz" | cut -f1)
        winrat_size=$(du "$compressed_file.rar" | cut -f1)

        echo "$file $(du -h $file | cut -f1) $(du -h "$compressed_file.gz" | cut -f1) $(du -h "$compressed_file.rar" | cut -f1) $(echo "scale=4; $gzip_size / $file_size" | bc) $(echo "scale=4; $winrat_size / $file_size" | bc) $(echo "scale=4; $gzip_size / $winrat_size" | bc)"
    done
) | column -t