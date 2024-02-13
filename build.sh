rm -rf build/pkg && \
cd magic && \
wasm-pack build --target web && \
cd .. && \
mv magic/pkg build/ && \
cd build && \
python3 -m http.server 8000
