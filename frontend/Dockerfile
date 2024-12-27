# Stage 1: Build the React app
FROM node:20 AS builder

WORKDIR /usr/src/app

COPY package.json yarn.lock ./

RUN yarn install

COPY . .

RUN yarn build

# Stage 2: Serve the app with Nginx
FROM nginx

COPY --from=builder usr/src/app/build /usr/share/nginx/html

# 复制自定义的 Nginx 配置文件到 Nginx 默认配置位置
COPY ./nginx/default.conf /etc/nginx/conf.d/default.conf

EXPOSE 18888

CMD ["nginx", "-g", "daemon off;"]
