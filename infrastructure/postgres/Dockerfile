FROM postgres:18.3-alpine
ENV POSTGRES_DB=todo
ENV POSTGRES_USER=todo
ENV POSTGRES_PASSWORD=password
COPY ./init.sql /docker-entrypoint-initdb.d/