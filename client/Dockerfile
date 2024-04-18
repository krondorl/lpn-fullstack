FROM node:21-alpine
WORKDIR /app
COPY package*.json .
RUN npm install
COPY . .
RUN npm run build
CMD [ "npm", "run", "preview"]

# Commands
#
# Build
# docker buildx build -t lpn-client:v01 .
#
# Run
# docker run -p 3000:3000 lpn-client:v01