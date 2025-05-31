FROM node:20-alpine
# FROM node:20-alpine as builder
COPY package*.json ./
RUN npm ci
# RUN npm ci --omit-dev
COPY . .
RUN npm run build

# FROM node:20-alpine
# COPY --from=builder build node_modules package*.json ./

EXPOSE 3000
CMD [ "node", "build" ]
