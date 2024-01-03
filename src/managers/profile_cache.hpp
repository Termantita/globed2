#pragma once
#include <defs.hpp>
#include <data/types/gd.hpp>
#include <util/sync.hpp>

class ProfileCacheManager : GLOBED_SINGLETON(ProfileCacheManager) {
public:
    ProfileCacheManager();

    void insert(const PlayerAccountData& data);
    std::optional<PlayerAccountData> getData(int32_t accountId);
    void clear();

    // gather player's icons and call `setOwnData`;
    void setOwnDataAuto();
    void setOwnData(const PlayerIconData& data);
    PlayerIconData getOwnData();

    bool pendingChanges = false;
private:
    std::unordered_map<int32_t, PlayerAccountData> cache;
    PlayerIconData ownData;
};