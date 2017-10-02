import { Epic } from "redux-observable"

import {
    MindStreamAction, READ_FEED, reloadUnreadedFeeds, loadUnreadedFeedsSuccess, loadUnreadedFeedsError,
    readFeedSuccess, readFeedError,
} from "./MindStreamActions"
import { GlobalState } from "../app/AppState"
import * as Api from "../Api"

export const loadUnreadedFeedsEpic: Epic<MindStreamAction, GlobalState> = (action$, state) => action$.ofType("LOAD_UNREADED_FEEDS")
    .mergeMap(action =>
        Api.loadUnreadedFeeds()
            .then(loadUnreadedFeedsSuccess)
            .catch(loadUnreadedFeedsError)
    )

export const reloadedUnreadedFeedsEpic: Epic<MindStreamAction, GlobalState> = (action$, state) => action$.ofType("RELOAD_UNREADED_FEEDS")
    .mergeMap(action =>
        Api.loadUnreadedFeeds()
            .then(loadUnreadedFeedsSuccess)
            .catch(loadUnreadedFeedsError)
    )

export const reloadUnreadedFeedsEpic: Epic<MindStreamAction, GlobalState> = (action$, state) => action$
    .filter(action => action.type === "READ_FEED_SUCCESS" && state.getState().mindStream.feeds.length === 0)
    .map(reloadUnreadedFeeds)

export const readFeedEpic: Epic<MindStreamAction, GlobalState> = (action$, state) => action$.ofType("READ_FEED")
    .mergeMap((action: READ_FEED) =>
        Api.feedReaction(action.feed, action.reaction)
            .then(readFeedSuccess)
            .catch(readFeedError)
    )
