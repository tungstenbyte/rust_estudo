FROM golang:1.20-alpine AS novo-exemplo-palm-pay
LABEL stage=gobuilder
RUN apk --no-cache add build-base git mercurial gcc ca-certificates openssh
RUN apk update

ADD git/id_rsa /root/.ssh/id_rsa
ADD git/known_hosts /root/.ssh/known_hosts
ADD git/authorized_keys /root/.ssh/authorized_keys
ADD git/id_ed25519 /root/.ssh/id_ed25519

RUN chmod 0700 /root/.ssh/id_rsa
RUN chmod 0700 /root/.ssh/id_ed25519
RUN echo "Host github.com\n\tStrictHostKeyChecking no\n" >> /root/.ssh/config
RUN git config --global url.ssh://git@github.com/.insteadOf https://github.com/

ADD . /src
RUN cd /src && go build -o locality

# final stage
FROM alpine AS final_image
LABEL stage=minimalbuilder
WORKDIR /go/src/microservices
COPY ./config/ ./config/
COPY ./ssl/server-key.pem ./ssl/
COPY ./ssl/server-cert.pem ./ssl/
COPY --from=novo-exemplo-palm-pay /src/novo-exemplo-palm-pay /go/src/microservices/
ENTRYPOINT ./novo-exemplo-palm-pay

