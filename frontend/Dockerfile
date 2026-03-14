FROM node:24-slim AS builder
 
WORKDIR /app

# Install pnpm
RUN npm install -g pnpm@latest-10

# Cache node_modules
COPY package.json pnpm-lock.yaml ./
RUN pnpm install

# Build
COPY . ./
RUN pnpm run build

# Runtime
FROM nginx:1.29-alpine AS runtime

RUN rm /etc/nginx/conf.d/default.conf
COPY nginx.conf /etc/nginx/nginx.conf
COPY --from=builder /app/build /var/www/html

EXPOSE 80

ENTRYPOINT ["nginx", "-g", "daemon off;"]
