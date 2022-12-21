for i in *.cairo; do
    cairo-compile $i --output "${i%%.*}".json
done