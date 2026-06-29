class LRUCache {
private:
    int capacity;

    std::list<std::pair<int, int>> cache;

    std::unordered_map<
        int,
        std::list<std::pair<int, int>>::iterator
    > positions;

public:
    LRUCache(int capacity)
        : capacity(capacity) {}

    int get(int key) {
        auto it = positions.find(key);

        if (it == positions.end()) {
            return -1;
        }

        cache.splice(cache.begin(), cache, it->second);

        return it->second->second;
    }

    void put(int key, int value) {
        if (capacity == 0) {
            return;
        }

        auto it = positions.find(key);

        if (it != positions.end()) {
            it->second->second = value;
            cache.splice(cache.begin(), cache, it->second);
            return;
        }

        if (cache.size() == capacity) {
            positions.erase(cache.back().first);
            cache.pop_back();
        }

        cache.emplace_front(key, value);
        positions[key] = cache.begin();
    }
};