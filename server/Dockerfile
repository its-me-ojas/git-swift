FROM alpine:latest


WORKDIR /app
COPY server .
COPY .env .

RUN chmod +x ./server

CMD ["./server"]