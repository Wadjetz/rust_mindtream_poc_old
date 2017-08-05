import { Feed, Reaction } from "./feeds/Feed"
import * as router from "./router"

const BASE_URI = "http://localhost:8000"

interface LoginResponse {
    login: string
}

function fetchOptions(query: string) {
    return {
        method: "POST",
        body: JSON.stringify({
            query: query.replace(/\s\s*/g, " "),
        }),
        headers: {
            "Content-Type": "application/json"
        }
    }
}

export function login(email: string, password: string): Promise<LoginResponse> {
    const options = fetchOptions(`mutation {
        login(email: "${email}", password: "${password}")
    }`)
    return fetch(`${BASE_URI}/graphql`, options)
    .then(response => response.json())
    .then(success)
}

export function loadUnreadedFeeds(token: string): Promise<Feed[]> {
    const options = fetchOptions(`
        query {
            auth(token: "${token}") {
                unreadedFeeds {
                    uuid
                    url
                    readable {
                        url
                        title
                        content
                        excerpt
                        leadImageUrl
                    }
                    rss {
                        title
                        content
                        summary
                    }
                }
            }
        }
    `)
    return fetch(`${BASE_URI}/graphql`, options)
    .then(response => response.json())
    .then(success)
    .then(result => result.auth.unreadedFeeds)
}

export function readFeed(token: string, feed: Feed, reaction: Reaction): Promise<void> {
    const options = fetchOptions(`
        mutation {
            auth(token: "${token}") {
                feedReaction(feedUuid: "${feed.uuid}", reaction: "${reaction}")
            }
        }
    `)
    return fetch(`${BASE_URI}/graphql`, options)
    .then(response => response.json())
    .then(success)
}

export function success(result: any) {
    if (result.errors) {
        if (result.errors.find((e: any) => e.message === "invalid token")) {
            router.push("/login")
        }
        throw { errors: result.errors }
    } else {
        return result.data
    }
}
