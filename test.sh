source .pyenv/bin/activate
maturin develop -r
python3 -m test.py
