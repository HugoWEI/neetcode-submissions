class LRUCache {
private:
    vector<pair<int, int>> m_cache;
    int m_capacity;

public:
    LRUCache(int capacity) {
        this->m_capacity = capacity;
    }
    
    int get(int key) {
        for (int i = 0; i < m_cache.size(); i++)
        {
            if (m_cache[i].first == key)
            {
                pair<int, int> tmp = m_cache[i];
                m_cache.erase(m_cache.begin() + i);
                m_cache.push_back(tmp);
                return tmp.second;
            }
        }

        return -1;
    }
    
    void put(int key, int value) {
        for (int i = 0; i < m_cache.size(); i++)
        {
            if (m_cache[i].first == key)
            {
                m_cache.erase(m_cache.begin() + i);
                m_cache.push_back({key, value});
                return;
            }
        }

        if (m_cache.size() == m_capacity)
        {
            m_cache.erase(m_cache.begin());
        }

        m_cache.push_back({key, value});
    }
};
