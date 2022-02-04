FROM docker.io/library/node:14.17.0-alpine
WORKDIR ./apollo-server
RUN apk add --no-cache bash
COPY package.json gateway.js wait-for-it.sh ./
RUN npm install
RUN chmod +x wait-for-it.sh
# wait until all downstream services start
CMD ./wait-for-it.sh users:8000 -s -- \
    ./wait-for-it.sh leagues:8001 -s -- \
    ./wait-for-it.sh teams:8002 -s -- \
    ./wait-for-it.sh players:8003 -s -- \
    npm run start-gateway
EXPOSE 4000
