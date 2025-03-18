package rfastutil.longs;

public class Long2ObjectHashMap<V> {

    protected transient Entry<V>[] table;

    private static final int DEFAULT_INITIAL_SIZE = 16;

    protected int size;

    public Long2ObjectHashMap() {
        this(DEFAULT_INITIAL_SIZE);
    }

    public Long2ObjectHashMap(int initialCapacity) {
        this.size = 0;
        this.table = new Entry[initialCapacity];
    }

    // Native methods declaration
    private static native Object get(Entry<Object>[] arr, long key);
    private static native Object put(Entry<Object>[] arr, long key, Object value);
    private static native Object remove(Entry<Object>[] arr, long key);
    private static native boolean containsKey(Entry<Object>[] arr, long key);
    private static native void clear(Entry<Object>[] arr);

    // Public API
    @SuppressWarnings("unchecked")
    public V get(long key) {
        return (V) Long2ObjectHashMap.get((Entry<Object>[]) table, key);
    }
    @SuppressWarnings("unchecked")
    public V put(long key, V value) {
        V oldValue = (V) Long2ObjectHashMap.put((Entry<Object>[]) table, key, value);
        if (oldValue == null) {
            size++;
        }
        return oldValue;
    }
    @SuppressWarnings("unchecked")
    public V remove(long key) {
        V oldValue = (V) Long2ObjectHashMap.remove((Entry<Object>[]) table, key);
        if (oldValue != null) {
            size--;
        }
        return oldValue;
    }
    @SuppressWarnings("unchecked")
    public boolean containsKey(long key) {
        return Long2ObjectHashMap.containsKey((Entry<Object>[]) table, key);
    }
    @SuppressWarnings("unchecked")
    public void clear() {
        Long2ObjectHashMap.clear((Entry<Object>[]) table);
        size = 0;
    }
    public int size() {
        return size;
    }

    public boolean isEmpty() {
        return size == 0;
    }

    // Entry class (nested)
    public static class Entry<V> {
        public long key;
        public V value;

        public Entry(long key, V value) {
            this.key = key;
            this.value = value;
        }
    }

    // Load the native library
    static {
        System.loadLibrary("rfastutil");
    }
}
