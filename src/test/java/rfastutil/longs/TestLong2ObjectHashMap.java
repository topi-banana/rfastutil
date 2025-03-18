package rfastutil.longs;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

public class TestLong2ObjectHashMap {

    @Test
    public void testPutGetRemove() {
        Long2ObjectHashMap<String> map = new Long2ObjectHashMap<>();
        // Put
        map.put(1L, "one");
        map.put(2L, "two");
        map.put(3L, "three");

        // Get
        assertEquals("one", map.get(1L));
        assertEquals("two", map.get(2L));
        assertEquals("three", map.get(3L));
        assertNull(map.get(4L));

        // ContainsKey
        assertTrue(map.containsKey(1L));
        assertTrue(map.containsKey(2L));
        assertTrue(map.containsKey(3L));
        assertFalse(map.containsKey(4L));

        // Remove
        assertEquals("two", map.remove(2L));
        assertNull(map.get(2L));
        assertFalse(map.containsKey(2L));

        // Put after remove
        map.put(2L, "new_two");
        assertEquals("new_two", map.get(2L));
        assertTrue(map.containsKey(2L));

        // Clear
        map.clear();
        assertNull(map.get(1L));
        assertNull(map.get(2L));
        assertNull(map.get(3L));
        assertFalse(map.containsKey(1L));
        assertFalse(map.containsKey(2L));
        assertFalse(map.containsKey(3L));
    }
}

