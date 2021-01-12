FROM alpine:3:12

RUN set -x \
 && apk add --no-cache \
        ca-certificates \
        curl \
        ffmpeg \
        gnupg \
        python3 \
 && curl -Lo /usr/local/bin/youtube-dl https://yt-dl.org/downloads/latest/youtube-dl \
 && curl -Lo youtube-dl.sig https://yt-dl.org/downloads/latest/youtube-dl.sig \
 && gpg --keyserver keyserver.ubuntu.com --recv-keys '7D33D762FD6C35130481347FDB4B54CBA4826A18' \
 && gpg --keyserver keyserver.ubuntu.com --recv-keys 'ED7F5BF46B3BBED81C87368E2C393E0F18A9236D' \
 && gpg --verify youtube-dl.sig /usr/local/bin/youtube-dl \
 && chmod a+rx /usr/local/bin/youtube-dl \
 && ln -s /usr/bin/python3 /usr/bin/python \
 && rm youtube-dl.sig \
 && apk del curl gnupg \
    # Create directory to hold downloads.
 && mkdir /data\
 && chmod a+rw /data \
    # Sets up cache.
 && mkdir /.cache \
 && chmod 777 /.cache

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt

COPY toobdoonlood /data
COPY Rocket.toml /data

WORKDIR /data

ENTRYPOINT ["toobdoonlood"]