# NerdTree website
This repo contains the source code for the NerdTree website. It is MVP and currently has basic features like blogging, user accounts etc.

# Building
- Clone this repo
- Make `.env` containing the following:
```env
POSTGRES_USER=nerd
POSTGRES_PASSWORD=nerd
POSTGRES_DB=nerd
```
- Make `server/.env` containing the following:
```env
JWT_SECRET_KEY=YOUR SECRET KEY HERE
DATABASE_URL=postgres://nerd:nerd@database:5432/nerd?sslmode=disable
SMTP_EMAIL=YOUR SMTP EMAIL HERE
SMTP_PASSWORD=YOUR SMTP PASSWORD HERE
SMTP_SERVER=YOUR SMTP SERVER HERE
DISCORD_BOT_TOKEN=YOUR SECRET DISCORD BOT TOKEN HERE
IMAGE_PATH=avatar
WEBSITE_LINK=http://localhost:5173
```
(make sure to fill the necessary informations)
- Run `docker-compose up --build`
- When the server is running, go to `server` and run `diesel migration run --database-url postgres://nerd:nerd@localhost/nerd`
- Restart the server
- Check if everything's working by running the frontend by first running `yarn` to install dependencies, then `yarn dev` and trying to create an account.

# Contributing
See [CONTRIBUTING.md](./CONTRIBUTING.md)
