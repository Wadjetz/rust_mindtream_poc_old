import { Feed } from "../feeds/Feed"

export const LOAD_UNREADED_FEEDS = "LOAD_UNREADED_FEEDS"
export const loadUnreadedFeeds = () => ({ type: LOAD_UNREADED_FEEDS })

export const LOAD_UNREADED_FEEDS_SUCCESS = "LOAD_UNREADED_FEEDS_SUCCESS"
export const loadUnreadedFeedsSuccess = (feeds: Feed[]) => ({ type: LOAD_UNREADED_FEEDS_SUCCESS, feeds })

export const LOAD_UNREADED_FEEDS_ERROR = "LOAD_UNREADED_FEEDS_ERROR"
export const loadUnreadedFeedsError = (error: any) => ({ type: LOAD_UNREADED_FEEDS_ERROR, error })


export const READ_FEED = "READ_FEED"
export const readFeed = () => ({ type: READ_FEED })

export const READ_FEED_SUCCESS = "READ_FEED_SUCCESS"
export const readFeedSuccess = (feed: Feed) => ({ type: READ_FEED_SUCCESS, feed })

export const READ_FEED_ERROR = "READ_FEED_ERROR"
export const readFeedError = (error: any) => ({ type: READ_FEED_ERROR, error })
