#pragma once
#include <defs/all.hpp>
#include <Geode/utils/web.hpp>
#include <Geode/ui/TextInput.hpp>
#include "chat_cell.hpp"

using namespace geode::prelude;

class GlobedChatListPopup : public geode::Popup<> {
protected:
	bool setup() override;

    CCMenuItemSpriteExtra* reviewButton;
    CCScale9Sprite* background;
    ScrollLayer* scroll = nullptr;
    CCLayer* layer2;
    geode::TextInput* inp;
    CCMenu* menu;
    std::vector<GlobedUserChatCell*> messageCells;

    float nextY = 0.f;
    int messages = 0;

    void onChat(CCObject* sender);
    void onClose(CCObject* sender) override;

    void updateChat(float dt);

    virtual void keyBackClicked() override;

public:
	static GlobedChatListPopup* create();
    void createMessage(int accountID, std::string message);
};