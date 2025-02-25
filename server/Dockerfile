FROM alpine:latest

WORKDIR /app

COPY dms-backend /app/
COPY .env /app/

RUN chmod +x /app/dms-backend

CMD ["./dms-backend"]
