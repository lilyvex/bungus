<div align="center">
    <h1>Bungus</h1>

A funny [Markov Chain](https://en.wikipedia.org/wiki/Markov_chain) Discord bot

</div>

> [!NOTE]
> Bungus can use more resources than expected under heavy load 
> (e.g. lots of messages being sent, commands being run, etc.).
> Ensure the machine you run it on has the adequate resources.

## Setup

First, go to [Discord's developer portal](https://discord.com/developers/applications) and create a new app. In the
bot tab, make sure to enable the "Message Content Intent". Ensure that when you invite the bot, it has the "Send Messages"
permission at the bare minimum. Secondly, Clone this repository (`git clone https://github.com/lilyvex/bungus`),
then build the project (`cargo build --release`). Rename the `.env.example` file to `.env` and ensure its contents match
your bot's token and associated guild ID if it is server specific.

## Direct Database Modification

> [!WARNING]
> This section is for work-in-progress code, it may be outdated.

> [!CAUTION]
> This is unsupported and may break between updates.

Bungus' database has the following structure:

    Servers -> Channels -> Messages
            |
            |> Weights

In the `weights` table for the associated `server` table you would like to modify, you can modify the weight of any word
Bungus knows, or even add new words and link them to existing ones.

<div align="center">
<hr/>

Made with ❤️ by [Lily](https://github.com/lilyvex) and [various contributors](https://github.com/lilyvex/bungus/graphs/contributors).

</div>