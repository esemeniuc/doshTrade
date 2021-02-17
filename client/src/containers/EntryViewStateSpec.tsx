const EntryViewStateSpec = {
    initialState: "blank",
    states: {
        blank: {
            ENTER_TICKER: "enteringTicker",
        },
        enteringTicker: {
            TICKER_FETCH_SUCCESS: "selectingExpirationAndStrategy",
            TICKER_FETCH_FAILURE: "enteringTicker",
        },
        selectingExpirationAndStrategy: {
            EXPIRATION_AND_STRATEGY_SELECT_SUCCESS: "generatingTrade",
            ENTER_TICKER: "enteringTicker",
            ERASE_TICKER: "blank",
        },
        presentingGeneratedTrade: {
            SELECT_EXPIRATION_AND_STRATEGY: "selectingExpirationAndStrategy",
            ENTER_TICKER: "enteringTicker",
            RESET: "blank",
        },
    },
}

export default EntryViewStateSpec
