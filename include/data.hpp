#pragma once

namespace globed::data {

	// Mid class to access fields on the packets. This is intrinsically related with the send and receive callbacks.
	template<typename T>
	class FieldAccesor {
		T set();
		void get(T);
	};
	
	enum class SendType;
	
	enum class ReceiveType;
	
	enum class FieldType;

	template<typename T = FieldType>
	class FieldMeta;
	
	// Register a mod. This shall be preferrably called inside $on_mod(Loaded).
	template<typename T = FieldType>
	void registerMod(geode::Mod mod, std::function<void(FieldAccesor<T>*)> sendCallback);
	
	// Register a field.
	template<typename T = FieldType>
	void registerDataField(geode::Mod mod, SendType packetType, FieldMeta<T> field);

	// Register a send callback. This callback will be called every time a packet which matches with ReceiveType is received.
	void registerReceiveCallback(geode::Mod mod, ReceiveType packetType);
}

namespace globed::data {

}