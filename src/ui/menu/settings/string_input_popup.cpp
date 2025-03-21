#include "string_input_popup.hpp"

bool StringInputPopup::setup(std::function<void(std::string_view)>) {
    // Build<geode::InputNode>::create(POPUP_WIDTH * 0.8f, "", "chatFont.)
    GLOBED_UNIMPL("StringInputPopup")

    return true;
}

StringInputPopup* StringInputPopup::create(std::function<void(std::string_view)> function) {
    auto ret = new StringInputPopup;
    if (ret->initAnchored(POPUP_WIDTH, POPUP_HEIGHT, function)) {
        ret->autorelease();
        return ret;
    }

    delete ret;
    return nullptr;
}