"use strict";
var __makeTemplateObject = (this && this.__makeTemplateObject) || function (cooked, raw) {
    if (Object.defineProperty) { Object.defineProperty(cooked, "raw", { value: raw }); } else { cooked.raw = raw; }
    return cooked;
};
exports.__esModule = true;
var react_1 = require("react");
var Container_1 = require("@material-ui/core/Container");
var styled_components_1 = require("styled-components");
var client_1 = require("@apollo/client");
var react_hook_form_1 = require("react-hook-form");
var GeneratedResults_1 = require("../components/GeneratedResults");
var graphql_macro_1 = require("graphql.macro");
var react_use_1 = require("react-use");
var EntryViewStateSpec_1 = require("./EntryViewStateSpec");
var useStateMachine_1 = require("../components/useStateMachine");
var GET_CURRENT_PRICE_QUERY = graphql_macro_1.loader("../graphql/getCurrentPrice.gql");
var Title = styled_components_1["default"].h2(templateObject_1 || (templateObject_1 = __makeTemplateObject(["\n  font-size: 2.5em;\n  text-align: left;\n  color: black;\n"], ["\n  font-size: 2.5em;\n  text-align: left;\n  color: black;\n"])));
var GeneratorForm = styled_components_1["default"].form(templateObject_2 || (templateObject_2 = __makeTemplateObject(["\n    font-size: 1em;\n"], ["\n    font-size: 1em;\n"])));
// Base class for 
var DoshInput = styled_components_1["default"].input(templateObject_3 || (templateObject_3 = __makeTemplateObject(["\n    display: block;\n    width: 100%;\n    box-sizing: border-box;\n    padding-left: 10px;\n    padding-top: 10px;\n    padding-bottom: 10px;\n    margin-bottom: 5px;\n    border-color: gray;\n    border-width: 3px;\n    border-radius: 7px;\n"], ["\n    display: block;\n    width: 100%;\n    box-sizing: border-box;\n    padding-left: 10px;\n    padding-top: 10px;\n    padding-bottom: 10px;\n    margin-bottom: 5px;\n    border-color: gray;\n    border-width: 3px;\n    border-radius: 7px;\n"])));
var TickerSearchInput = styled_components_1["default"](DoshInput)(templateObject_4 || (templateObject_4 = __makeTemplateObject(["\n    text-transform: uppercase;\n    &::placeholder {\n        text-transform: none;\n    }\n"], ["\n    text-transform: uppercase;\n    &::placeholder {\n        text-transform: none;\n    }\n"])));
var DoshSelect = styled_components_1["default"].select(templateObject_5 || (templateObject_5 = __makeTemplateObject(["\n    display: block;\n    padding-left: 10px;\n    padding-top: 10px;\n    padding-bottom: 10px;\n\twidth: 100%;\n    margin-bottom: 5px;\n\tbox-sizing: border-box;\n    border-width: 3px;\n    border-radius: 7px;\n\t-moz-appearance: none;\n\t-webkit-appearance: none;\n\tappearance: none;\n\tbackground-color: white;\n\tbackground-image: url('data:image/svg+xml;charset=US-ASCII,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%22292.4%22%20height%3D%22292.4%22%3E%3Cpath%20fill%3D%22%23007CB2%22%20d%3D%22M287%2069.4a17.6%2017.6%200%200%200-13-5.4H18.4c-5%200-9.3%201.8-12.9%205.4A17.6%2017.6%200%200%200%200%2082.2c0%205%201.8%209.3%205.4%2012.9l128%20127.9c3.6%203.6%207.8%205.4%2012.8%205.4s9.2-1.8%2012.8-5.4L287%2095c3.5-3.5%205.4-7.8%205.4-12.8%200-5-1.9-9.2-5.5-12.8z%22%2F%3E%3C%2Fsvg%3E');\n\tbackground-repeat: no-repeat, repeat;\n\tbackground-position: right .7em top 50%, 0 0;\n    background-size: .65em auto, 100%;\n"], ["\n    display: block;\n    padding-left: 10px;\n    padding-top: 10px;\n    padding-bottom: 10px;\n\twidth: 100%;\n    margin-bottom: 5px;\n\tbox-sizing: border-box;\n    border-width: 3px;\n    border-radius: 7px;\n\t-moz-appearance: none;\n\t-webkit-appearance: none;\n\tappearance: none;\n\tbackground-color: white;\n\tbackground-image: url('data:image/svg+xml;charset=US-ASCII,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%22292.4%22%20height%3D%22292.4%22%3E%3Cpath%20fill%3D%22%23007CB2%22%20d%3D%22M287%2069.4a17.6%2017.6%200%200%200-13-5.4H18.4c-5%200-9.3%201.8-12.9%205.4A17.6%2017.6%200%200%200%200%2082.2c0%205%201.8%209.3%205.4%2012.9l128%20127.9c3.6%203.6%207.8%205.4%2012.8%205.4s9.2-1.8%2012.8-5.4L287%2095c3.5-3.5%205.4-7.8%205.4-12.8%200-5-1.9-9.2-5.5-12.8z%22%2F%3E%3C%2Fsvg%3E');\n\tbackground-repeat: no-repeat, repeat;\n\tbackground-position: right .7em top 50%, 0 0;\n    background-size: .65em auto, 100%;\n"])));
var ActionButton = styled_components_1["default"].input(templateObject_6 || (templateObject_6 = __makeTemplateObject(["\n    display: block;\n    height: 40px;\n    box-sizing: border-box;\n    padding-left: 10px;\n    padding-top: 12px;\n    padding-bottom: 12px;\n    margin-bottom: 5px;\n    border: none;\n    border-radius: 7px;\n    color: white;\n"], ["\n    display: block;\n    height: 40px;\n    box-sizing: border-box;\n    padding-left: 10px;\n    padding-top: 12px;\n    padding-bottom: 12px;\n    margin-bottom: 5px;\n    border: none;\n    border-radius: 7px;\n    color: white;\n"])));
var GenerateButton = styled_components_1["default"](ActionButton)(templateObject_7 || (templateObject_7 = __makeTemplateObject(["\n    width: 70%;\n    background-color: black;\n    &:disabled {\n        opacity: 0.3;\n    }\n"], ["\n    width: 70%;\n    background-color: black;\n    &:disabled {\n        opacity: 0.3;\n    }\n"])));
var ResetButton = styled_components_1["default"](ActionButton)(templateObject_8 || (templateObject_8 = __makeTemplateObject(["\n    margin-top: 20px;\n    width: 100%;\n    background-color: black;\n"], ["\n    margin-top: 20px;\n    width: 100%;\n    background-color: black;\n"])));
var PriceLabel = styled_components_1["default"].div(templateObject_9 || (templateObject_9 = __makeTemplateObject(["\n    text-align: left;\n    width: 100%;\n    padding-top: 20px;\n    padding-left: 10px;\n    padding-bottom: 12px;\n"], ["\n    text-align: left;\n    width: 100%;\n    padding-top: 20px;\n    padding-left: 10px;\n    padding-bottom: 12px;\n"])));
var GeneratedResultsFrame = styled_components_1["default"].div(templateObject_10 || (templateObject_10 = __makeTemplateObject(["\n    background-color: gainsboro;\n    min-height: 300px;\n    width: 100%;\n    margin-top: 20px;\n    display: flex;\n    flex-direction: column;\n    justify-content: center;\n    align-items: center;\n"], ["\n    background-color: gainsboro;\n    min-height: 300px;\n    width: 100%;\n    margin-top: 20px;\n    display: flex;\n    flex-direction: column;\n    justify-content: center;\n    align-items: center;\n"])));
var EXPIRATION_PLACEHOLDER = "Expiration Date";
var STRATEGY_PLACEHOLDER = "Strategy";
function EntryView() {
    var _a = react_1.useState(false), submitted = _a[0], setSubmitted = _a[1];
    var _b = react_1.useState(''), debouncedTicker = _b[0], setDebouncedTicker = _b[1];
    var _c = useStateMachine_1.useStateMachine(EntryViewStateSpec_1["default"]), currentState = _c[0], sendEvent = _c[1];
    var onGenerate = function (formData) {
        console.log("onGenerate called");
        setSubmitted(true);
    };
    var onAfterReset = function (e) {
        setSubmitted(false);
    };
    var _d = react_hook_form_1.useForm({
        mode: "onChange"
    }), register = _d.register, handleSubmit = _d.handleSubmit, watch = _d.watch, errors = _d.errors, trigger = _d.trigger;
    var ticker = watch(["ticker"]).ticker;
    var expiration = watch(["expiration"]).expiration;
    var strategy = watch(["strategy"]).strategy;
    var _e = client_1.useQuery(GET_CURRENT_PRICE_QUERY, { variables: { ticker: debouncedTicker } }), data = _e.data, loading = _e.loading, error = _e.error;
    var priceString = (ticker && data) ? data.price : "$";
    if (currentState === 'blank' && ticker) {
        sendEvent("ENTER_TICKER");
    }
    else if (currentState === 'enteringTicker' && data && !error) {
        sendEvent("TICKER_FETCH_SUCCESS");
    }
    else if (currentState === 'selectingExpirationAndStrategy' && !ticker) {
        sendEvent("ERASE_TICKER");
    }
    react_use_1.useDebounce(function () {
        ticker && setDebouncedTicker(ticker);
        console.log('ticker: ', ticker);
    }, 350, [ticker]);
    var isExpirationAndStrategySelectable = currentState == "selectingExpirationAndStrategy" || currentState == "presentingGeneratedTrade";
    var isSubmitButtonEnabled = currentState == "selectingExpirationAndStrategy" && expiration !== EXPIRATION_PLACEHOLDER && strategy !== STRATEGY_PLACEHOLDER;
    return (react_1["default"].createElement(Container_1["default"], { component: "main", maxWidth: "sm", style: {
            backgroundColor: 'white',
            display: 'flex',
            flexDirection: 'column',
            alignContent: 'flex-start'
        } },
        react_1["default"].createElement(Title, null, " Option Analysis "),
        react_1["default"].createElement(GeneratorForm, { onSubmit: handleSubmit(onGenerate), onReset: onAfterReset },
            react_1["default"].createElement(TickerSearchInput, { name: "ticker", placeholder: "Ticker", ref: register({ required: true }) }),
            react_1["default"].createElement(PriceLabel, null,
                "Current price: ",
                priceString),
            react_1["default"].createElement(DoshSelect, { disabled: !isExpirationAndStrategySelectable, name: "expiration", defaultValue: EXPIRATION_PLACEHOLDER, ref: register },
                react_1["default"].createElement("option", { disabled: true }, " Expiration Date "),
                react_1["default"].createElement("option", null, "Apples"),
                react_1["default"].createElement("option", null, "Pears")),
            react_1["default"].createElement(DoshSelect, { disabled: !isExpirationAndStrategySelectable, name: "strategy", defaultValue: STRATEGY_PLACEHOLDER, ref: register },
                react_1["default"].createElement("option", { disabled: true }, " Strategy "),
                react_1["default"].createElement("option", null, "Buy Call"),
                react_1["default"].createElement("option", null, "Sell Call"),
                react_1["default"].createElement("option", null, "Buy Put"),
                react_1["default"].createElement("option", null, "Sell Put")),
            react_1["default"].createElement(GeneratedResultsFrame, null, submitted ?
                react_1["default"].createElement(GeneratedResults_1["default"], null) :
                react_1["default"].createElement(GenerateButton, { disabled: !isSubmitButtonEnabled, type: "submit", value: "Submit" })),
            submitted &&
                react_1["default"].createElement(ResetButton, { type: "reset", value: "Reset" }))));
}
exports["default"] = EntryView;
var templateObject_1, templateObject_2, templateObject_3, templateObject_4, templateObject_5, templateObject_6, templateObject_7, templateObject_8, templateObject_9, templateObject_10;
