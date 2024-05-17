#pragma once

#include <defs/util.hpp>
#include <data/types/user.hpp>

#include <asp/sync/Atomic.hpp>

class AdminManager : public SingletonBase<AdminManager> {
    friend class SingletonBase;

public:
    bool authorized();
    void setAuthorized(ComputedRole&& role);
    void deauthorize();
    ComputedRole& getRole();

private:
    asp::sync::AtomicBool authorized_;
    ComputedRole role = {};
};
