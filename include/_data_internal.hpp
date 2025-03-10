#include <data/packets/packet.hpp>
#include <util/singleton.hpp>

#include "data.hpp"

using namespace globed::data;

class APIDataManager : public SingletonBase<APIDataManager> {
public:
	void invokeSendCallbacks(Packet, SendType);

	void invokeReceiveCallbacks(auto, ReceiveType);
};