# frozen_string_literal: true
# 
# Tests comparing the output of Penguin::ObjectId with BSON::ObjectId.

require 'bson'

require_relative 'test_helper'

class TestBsonObjectId < Minitest::Test
  def test_compare
    bson_id = BSON::ObjectId.new
    penguin_id = Penguin::ObjectId.from_time(bson_id.to_time)
    assert_equal bson_id.generation_time, penguin_id.timestamp
  end

  def test_from_time
    t = Time.now - 1000
    # BSON::ObjectId.from_time doesn't respect the input time if `unique` is
    # true so only test with `unique: false`.
    bson_id = BSON::ObjectId.from_time(t, unique: false)
    penguin_id = Penguin::ObjectId.from_time(t, unique: false)
    assert_equal bson_id.generation_time, penguin_id.timestamp
    assert_equal bson_id._counter_part.to_i(16), penguin_id.counter
  end

  def test_from_string
    string = '64c13ab08edf48a008793cac'
    bson_id = BSON::ObjectId.from_string(string)
    penguin_id = Penguin::ObjectId.from_string(string)
    assert_equal bson_id.generation_time, penguin_id.timestamp
    assert_equal bson_id._counter_part.to_i(16), penguin_id.counter
    assert_equal bson_id._process_part.to_i(16), penguin_id.machine_id
  end
end