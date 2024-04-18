FROM node:21-alpine
WORKDIR /app
COPY package.json .
RUN npm install
COPY . .
RUN npm run build
CMD [ "npm", "run", "preview", "--", "--host", "0.0.0.0" ]

# Commands
#
# Build
# docker buildx build -t lpn-client:v01 .
#
# Run
# docker run -p 4200:4200 lpn-client:v01