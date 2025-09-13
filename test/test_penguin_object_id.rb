# frozen_string_literal: true

require "time"

require "test_helper"

class TestPenguinObjectId < Minitest::Test
  def test_from_time_unique
    t = Time.now - 1000
    100.times.reduce(Penguin::ObjectId.from_time(t, unique: true)) do |prev, _|
      id = Penguin::ObjectId.from_time(t, unique: true)
      assert_equal t.floor, id.timestamp
      assert_operator prev.counter, :<, id.counter
      assert_equal prev.machine_id, id.machine_id
      assert_operator prev.counter, :<, id.counter
      assert_operator prev, :<, id
      id
    end

  end

  def test_from_time_not_unique
    t = Time.now - 1000
    100.times.reduce(Penguin::ObjectId.from_time(t, unique: false)) do |prev, _|
      id = Penguin::ObjectId.from_time(t, unique: false)
      assert_equal t.floor, id.timestamp
      assert_equal 0, id.counter
      assert_equal prev.machine_id, id.machine_id
      assert_equal prev, id
      id
    end
  end

  def test_new
    100.times.reduce(Penguin::ObjectId.new) do |prev, _|
      id = Penguin::ObjectId.new
      assert_operator prev.counter, :<, id.counter
      assert_equal prev.machine_id, id.machine_id
      id
    end
  end

  def test_from_string
    strings = [
      '64c13ab08edf48a008793cac',
      '4e4d66343b39b68407000001',
    ]

    strings.each do |string|
      id = Penguin::ObjectId.from_string(string)
      assert_equal string, id.to_s
    end
  end

  def test_from_string_parts
    tables = [
      {
        string: '68c51d6691b330e4ed29c95d',
        timestamp: Time.parse('2025-09-13 07:29:42 UTC'),
        machine_id: 625776583917,
        counter: 2738525,
      },
      {
        string: '68c51d6691b330e4ed29c961',
        timestamp: Time.parse('2025-09-13 07:29:42 UTC'),
        counter: 2738529,
        machine_id: 625776583917,
      },
      {
        string: '68c51ec3e000353dcd156421',
        timestamp: Time.parse('2025-09-13 07:35:31 UTC'),
        counter: 1401889,
        machine_id: 962076163533,
      }
    ]

    tables.each do |table|
      id = Penguin::ObjectId.from_string(table[:string])
      assert_equal table[:timestamp], id.timestamp
      assert_equal table[:machine_id], id.machine_id
      assert_equal table[:counter], id.counter
      assert_equal table[:string], id.to_s
    end
  end
end
