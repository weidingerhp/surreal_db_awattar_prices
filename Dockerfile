FROM alpine:3.14.2

# Install dependencies
COPY target/release/awattar-prices /usr/local/bin/awattar-prices

ENTRYPOINT [ "/usr/local/bin/awattar-prices" ]