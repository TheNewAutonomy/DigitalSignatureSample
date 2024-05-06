# DigitalSignatureSample
Simple project to create a digital signature for an API call in Rust

# Generate a CSR and private key and use your private key
# A sample private key is included for unit tests
openssl req -newkey rsa:2048 -nodes -days 365000 -keyout client-key.pem -out client-req.pem
