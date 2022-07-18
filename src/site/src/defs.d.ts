/**
 * This is the base for the configs for the bot.
 */
export class Configs {
    /**
     * @property The bot's prefix
     */
    Prefix: string;

    /**
     * @property The IP for the server. Same as the url resolution
     */
    Server: string;

    /**
     * @property Database name
     */
    Database: string;

    /**
     * @property Database collection name
     */
    Collection: string;

    /**
     * @property The site's port
     * @type {number}
     */
    Port: number;
}
